export class Assertions {

    static is_true(val, msg) {
        if (typeof msg !== "string") {
            throw Error("no msg passed to assertion method");
        }
        if (val !== true) {
            throw Error("assertion failed: " + msg);
        }
    }

    static is_equal(valA, valB, msg) {
        this.is_true(valA === valB, msg);
    }

    static is_lt(valA, valB, msg) {
        this.is_true(valA < valB, msg);
    }

    static is_gt(valA, valB, msg) {
        this.is_true(valA > valB, msg);
    }

    static is_false(val, msg) {
        this.is_true(val === false, msg);
    }

    static not_null(obj, msg) {
        this.is_true(undefined !== obj && null !== obj, msg);
    }

    static member_is_undefined(obj, memberName, msg) {
        this.is_true(typeof obj[memberName] === "undefined", msg)
    }

    static member_is_null(obj, memberName, msg) {
        this.is_true(null === obj[memberName], msg);
    }

    static is_array(obj, msg) {
        this.is_true(Array.isArray(obj), msg);
    }

    static is_function(obj, msg) {
        this.is_true(typeof obj === "function", msg);
    }

    static is_object(obj, msg) {
        this.is_true(typeof obj === "object", msg);
    }

    static is_string(obj, msg) {
        this.is_true(typeof obj === "string", msg);
    }

    static is_number(obj, msg) {
        this.is_true(typeof obj === "number", msg);
    }

    static is_boolean(obj, msg) {
        this.is_true(typeof obj === "boolean", msg);
    }

    static is_instance_of(obj, obj_type, msg) {
        this.not_null(obj, msg);
        this.not_null(obj_type, "obj_type may not be null when calling Assertions.is_instance_of().");
        let ok = false;
        if (typeof obj_type === "string") {
            if (typeof obj === obj_type) {
                ok = true;
            } else if (obj.constructor && obj.constructor.name === obj_type) {
                ok = true;
            }
        } else {
            ok = obj instanceof obj_type;
        }
        this.is_true(ok, msg);
    }

    static is_array_of(obj, obj_type, msg) {
        this.is_array(obj, msg);
        for (let val of obj) {
            this.is_instance_of(val, obj_type, msg);
        }
    }

};
